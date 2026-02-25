-- Add up migration script here

-- สร้างตาราง Categories (หมวดหมู่)
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    type VARCHAR(10) NOT NULL CHECK (type IN ('income', 'expense')), 
    icon VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- สร้างตาราง Transactions (รายการรายรับ-รายจ่าย)
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    amount DECIMAL(12, 2) NOT NULL,           
    description TEXT,                 
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    transaction_date DATE NOT NULL DEFAULT CURRENT_DATE,
    user_id INTEGER, 
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO categories (name, type) VALUES 
('เงินเดือน', 'income'),
('ขายของ', 'income'),
('อาหาร', 'expense'),
('เดินทาง', 'expense'),
('ที่พัก', 'expense');