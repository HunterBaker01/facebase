from deepface import DeepFace
import psycopg

def enrol(img: str, img_name: str):
    conn = psycopg.connect("postgresql://localhost/family_photos")
    result = DeepFace.represent(
        img_path=img,
        model_name="ArcFace",
        detector_backend="retinaface",
        l2_normalize=True,
    )
    print(f"Registered {img_name}: {result}")
    conn.close()

enrol("../14bf5083-5a03-44e0-bb65-d6aae0aef276.JPG", "Hunter")
