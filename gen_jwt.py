import jwt  # Ensure this is importing the correct library
import datetime

# Your secret key must match exactly with the server's JWT_SECRET
secret_key = "secret"

# Create the token payload
payload = {
    "sub": "radu",  # Subject
    "exp": datetime.datetime.utcnow() + datetime.timedelta(days=30)  # Expiration time (30 days from now)
}

# Generate the token using HS256 algorithm
token = jwt.encode(payload, secret_key, algorithm="HS256")

print(token)
