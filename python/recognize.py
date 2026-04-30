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
    results = DeepFace.find(
        img_path=load_image(path),
        db_path=db_path,
        enforce_detection=False,
        silent=True,
    )

    output = [{"identity": row["identity"]} for df in results for _, row in df.iterrows()]

    print(json.dumps(output))

def enroll(path: str):
    DeepFace.register(img=path)
# img_path = sys.argv[1]
# recognition(img_path, "../data/")
enroll("../IMG_8882.jpg")
