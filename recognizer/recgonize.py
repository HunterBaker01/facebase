from deepface import DeepFace
import pillow_heif
from PIL import Image

pillow_heif.register_heif_opener()

def load_image(path: str) -> str:
    if path.lower().endswith(".heic"):
        image = Image.open(path)
        converted = path.rsplit(".", 1)[0] + ".jpg"
        image.save(converted, "JPEG")
        return converted
    return path

