# Base image with Python 3.9
FROM python:3.9-buster

#RUN apt-get -y update
RUN apt-get -y install git

# Create and set the working directory
WORKDIR /app

# Copy the requirements file and install the dependencies
COPY requirements.txt .
RUN pip install -r requirements.txt

# Copy the source code into the working directory
COPY . .

# Set the default command to start the application
CMD ["python", "gitpoll_foreground.py"]
# CMD ["bash"]
