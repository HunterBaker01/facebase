from deepface import DeepFace
import psycopg

def enrol(img: str, name: str, notes: str="None"):
    with psycopg.connect("postgresql://localhost/photos") as conn:
        with conn.cursor() as cur:
            result = DeepFace.represent(
                img_path=img,
                model_name="ArcFace",
                detector_backend="retinaface",
                l2_normalize=True,
            )
            cur.execute(
            "INSERT INTO people (name, notes) VALUES (%s, %s)",
            (name, notes))

enrol("../IMG_8882.jpg", "Hunter", "Son of Lorrie")
