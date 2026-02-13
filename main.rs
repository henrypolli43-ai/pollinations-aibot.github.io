<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Beautiful Page</title>
    <style>
        body {
            margin: 0;
            font-family: Arial, sans-serif;
            background: linear-gradient(to right, #f3f4f6, #d1d5db);
        }
        header {
            background: #4f46e5;
            color: white;
            padding: 20px;
            text-align: center;
        }
        .feature-cards {
            display: flex;
            justify-content: space-around;
            margin: 20px 0;
        }
        .card {
            background: white;
            border-radius: 10px;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
            padding: 20px;
            width: 30%;
            transition: transform 0.2s;
        }
        .card:hover {
            transform: scale(1.05);
        }
        footer {
            text-align: center;
            margin: 20px 0;
        }
        @media (max-width: 768px) {
            .feature-cards {
                flex-direction: column;
                align-items: center;
            }
            .card {
                width: 80%;
                margin-bottom: 20px;
            }
        }
    </style>
</head>
<body>
    <header>
        <h1>Welcome to My Beautiful HTML Page</h1>
    </header>
    <div class="feature-cards">
        <div class="card">
            <h2>Feature 1</h2>
            <p>Details about Feature 1.</p>
        </div>
        <div class="card">
            <h2>Feature 2</h2>
            <p>Details about Feature 2.</p>
        </div>
        <div class="card">
            <h2>Feature 3</h2>
            <p>Details about Feature 3.</p>
        </div>
    </div>
    <footer>
        <p>Statistics: 100% satisfaction rate!</p>
    </footer>
</body>
</html>