-- Hapus dalam urutan yang aman (tabel dependen dulu)
DROP TABLE IF EXISTS uptime_logs;
DROP TABLE IF EXISTS monitoring_services;

DROP TABLE IF EXISTS post_tags;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS tags;

DROP TABLE IF EXISTS project_tools;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS tools;

DROP TABLE IF EXISTS skills;
DROP TABLE IF EXISTS skill_categories;

DROP TABLE IF EXISTS users;
