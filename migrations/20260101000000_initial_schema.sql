-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Content types schema (flexible storage)
CREATE TABLE content_types (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT,
    schema_json JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Content entries (hybrid: IDs + JSONB)
CREATE TABLE entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content_type_id UUID NOT NULL REFERENCES content_types(id) ON DELETE CASCADE,
    author_id UUID NOT NULL,
    slug TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    data JSONB NOT NULL DEFAULT '{}',
    meta JSONB NOT NULL DEFAULT '{}',
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(content_type_id, slug)
);

-- Tags (relational many-to-many via junction)
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    slug TEXT UNIQUE NOT NULL
);

CREATE TABLE entry_tags (
    entry_id UUID REFERENCES entries(id) ON DELETE CASCADE,
    tag_id UUID REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (entry_id, tag_id)
);

-- Media assets
CREATE TABLE media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename TEXT NOT NULL,
    key TEXT UNIQUE NOT NULL,
    mime_type TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    width INT,
    height INT,
    uploaded_by UUID NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_entries_content_type ON entries(content_type_id);
CREATE INDEX idx_entries_author ON entries(author_id);
CREATE INDEX idx_entries_status ON entries(status);
CREATE INDEX idx_entries_published_at ON entries(published_at);
CREATE INDEX idx_entries_data_gin ON entries USING GIN(data);
CREATE INDEX idx_tags_slug ON tags(slug);

-- Automatic updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_content_types_updated_at BEFORE UPDATE ON content_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_entries_updated_at BEFORE UPDATE ON entries
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
