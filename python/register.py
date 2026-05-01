from deepface import DeepFace
import psycopg
from pgvector.psycopg import register_vector

def enrol(img: str, name: str, notes: str="None"):
    with psycopg.connect("postgresql://localhost/photos") as conn:
        with conn.cursor() as cur:
            register_vector(conn)
            result = DeepFace.represent(
                img_path=img,
                model_name="ArcFace",
                detector_backend="retinaface",
                l2_normalize=True,
            )

            embedding = result[0]["embedding"]
            cur.execute(
                "INSERT INTO people (name, notes) VALUES (%s, %s) "
                "ON CONFLICT (name) DO UPDATE SET notes = EXCLUDED.notes "
                "RETURNING id",
                (name, notes)
            )
            person_id = cur.fetchone()[0]
            cur.execute(
                "INSERT INTO face_embeddings (person_id, embedding, source_image_path) VALUES (%s, %s, %s)",
                (person_id, embedding, img)
            )
            print(f"{name} added successfully!")

enrol("../IMG_8882.jpg", "Hunter", "Son of Lorrie")
