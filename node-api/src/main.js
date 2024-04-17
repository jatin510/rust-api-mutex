const express = require('express');
const jwt = require('jsonwebtoken')
const app = express();

const ACCESS_TOKEN_SECRET = 'access_secret';
const REFRESH_TOKEN_SECRET = 'refresh_secret';

// Middleware to parse JSON bodies
app.use(express.json());

function authenticateToken(req, res, next) {
    // Retrieve token from the Authorization header
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1];

    if (token == null) {
        return res.sendStatus(401); // No token provided
    }

    jwt.verify(token, ACCESS_TOKEN_SECRET, (err, user) => {
        if (err) {
            return res.sendStatus(401); // Unauthorized access
        }

        req.user = user;
        next();
    });
}

function verifyRefreshToken(req, res, next) {
    const { token } = req.body;

    if (token == null) return res.sendStatus(401);
    // if (!refreshTokens.includes(token)) return res.sendStatus(403); // Token not found

    jwt.verify(token, REFRESH_TOKEN_SECRET, (err, user) => {
        if (err) return res.sendStatus(403); // Invalid token

        req.user = user;
        next();
    });
}


// Simulate a login endpoint
app.post('/login', (req, res) => {
    const { username, password } = req.body;

    // Validate username and password here
    // For demonstration, let's assume the credentials are valid
    if (username === 'user' && password === 'pass') {
        // Normally, tokens would be generated using a library like jsonwebtoken
        const accessToken = jwt.sign({ user: username }, ACCESS_TOKEN_SECRET, { expiresIn: '1d' });
        const refreshToken = jwt.sign({ user: username }, REFRESH_TOKEN_SECRET, { expiresIn: '7d' });

        // Send the tokens as a response
        res.json({
            accessToken,
            refreshToken
        });
    } else {
        res.status(401).json({ message: 'Invalid credentials' });
    }
});

app.get('/protected', authenticateToken, (req, res) => {
    // Example response that returns user data
    console.log('accessing protected route')
    res.json({ message: 'Welcome to the protected route!', user: req.user });
});

app.post('/refresh-access-token',
    verifyRefreshToken,
    (req, res) => {
        const { user } = req;
        const accessToken = jwt.sign({ username: user.username }, ACCESS_TOKEN_SECRET, { expiresIn: '1d' });
        const refreshToken = jwt.sign({ user: user.username }, REFRESH_TOKEN_SECRET, { expiresIn: '7d' });

        console.log('new refresh token generated');
        res.json({
            accessToken,
            refreshToken
        });
    });


// Start the server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
