from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker, declarative_base

# We use SQLite for Phase 1 dev to avoid Docker complexity if you haven't started it yet
SQLALCHEMY_DATABASE_URL = "sqlite:///./pratyaksh_dev.db"

engine = create_engine(SQLALCHEMY_DATABASE_URL, connect_args={"check_same_thread": False})
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
Base = declarative_base()
