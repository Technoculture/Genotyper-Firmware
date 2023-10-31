from pydantic import BaseModel


class Message(BaseModel):
    data: str
