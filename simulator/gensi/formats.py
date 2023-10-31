from datetime import datetime
from pydantic import BaseModel, Field

class Message(BaseModel):
    data: str

class Sample(BaseModel):
    category: str = Field(default="Genotyping")
    timestamp: datetime = Field(default_factory=datetime.now)
