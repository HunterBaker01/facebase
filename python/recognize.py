from deepface import DeepFace
import pillow_heif
from PIL import Image
import json, sys

pillow_heif.register_heif_opener()

def load_image(path: str) -> str:
    if path.lower().endswith(".heic"):
        image = Image.open(path)
        converted = path.rsplit(".", 1)[0] + ".jpg"
        image.save(converted, "JPEG")
        return converted
    return path

def recognition(path: str, db_path: str):
    
