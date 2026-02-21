-- 1 Tabel Users (Auth)
CREATE TABLE users (
    id BIGINT PRIMARY KEY AUTO_INCREMENT, -- Atau UUID v7 untuk distribusi
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) DEFAULT 'admin',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Index untuk login cepat
CREATE INDEX idx_users_username ON users(username);
--------------------------------------------------------------------------

-- 2 Tabel Skills (Dinormalisasi dengan Kategori)
-- Tabel Skill Categories
CREATE TABLE skill_categories (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(50) UNIQUE NOT NULL, -- e.g., "OS", "Network"
    slug VARCHAR(50) UNIQUE NOT NULL,
    sort_order INT DEFAULT 0
);

CREATE TABLE skills (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    category_id INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    level VARCHAR(20) NOT NULL, -- e.g., "Expert", "Advanced"
    icon_url VARCHAR(255),
    FOREIGN KEY (category_id) REFERENCES skill_categories(id) ON DELETE CASCADE
);
-- Index untuk filtering berdasarkan kategori
CREATE INDEX idx_skills_category ON skills(category_id);

--------------------------------------------------------------------------
-- 3 Tabel Projects & Tools (Many-to-Many)
CREATE TABLE tools (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) UNIQUE NOT NULL, -- e.g., "Ansible", "Docker"
    slug VARCHAR(100) UNIQUE NOT NULL,
    logo_url VARCHAR(255)
);

CREATE TABLE projects (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    role VARCHAR(100),
    problem TEXT,
    solution TEXT,
    result TEXT,
    diagram_url VARCHAR(255),
    is_featured BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
-- Index untuk URL SEO dan Featured
CREATE INDEX idx_projects_slug ON projects(slug);
CREATE INDEX idx_projects_featured ON projects(is_featured);

CREATE TABLE project_tools (
    project_id BIGINT NOT NULL,
    tool_id INT NOT NULL,
    PRIMARY KEY (project_id, tool_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (tool_id) REFERENCES tools(id) ON DELETE CASCADE
);

--------------------------------------------------------------------------
-- 4 Tabel Posts/Blog & Tags (Many-to-Many)
CREATE TABLE tags (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(50) UNIQUE NOT NULL,
    slug VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE posts (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    content LONGTEXT, -- Pisahkan konten besar jika perlu optimasi ekstrem
    type VARCHAR(50), -- Tutorial, Troubleshooting
    status VARCHAR(20) DEFAULT 'published', -- draft, published
    published_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Index untuk filtering status dan tanggal
CREATE INDEX idx_posts_status_date ON posts(status, published_at);
CREATE INDEX idx_posts_slug ON posts(slug);

CREATE TABLE post_tags (
    post_id BIGINT NOT NULL,
    tag_id INT NOT NULL,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

--------------------------------------------------------------------------
-- 5 Tabel Monitoring (Uptime) 
CREATE TABLE monitoring_services (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    endpoint_url VARCHAR(255) NOT NULL
);

CREATE TABLE uptime_logs (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    service_id INT NOT NULL,
    status VARCHAR(20), -- up, down
    response_time_ms INT,
    checked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (service_id) REFERENCES monitoring_services(id) ON DELETE CASCADE
);
-- Index penting untuk query timeline uptime
CREATE INDEX idx_uptime_service_time ON uptime_logs(service_id, checked_at);