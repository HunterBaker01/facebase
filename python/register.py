from hashlib import sha256
from deepface import DeepFace
from deepface.commons.package_utils import hashlib
import psycopg
from pgvector.psycopg import register_vector
from pathlib import Path

def image_info(img: str):
    return DeepFace.represent(
        img_path=img,
        model_name="ArcFace",
        detector_backend="retinaface",
        l2_normalize=True,
    )

def add_embedding(conn, person_id: int, embedding: list, img_path: str):
    with conn.cursor() as cur:
        cur.execute(
            "INSERT INTO face_embeddings (person_id, embedding, source_image_path) "
            "VALUES (%s, %s, %s) ",
            (person_id, embedding, img_path)
        )

def add_photo(conn, img_path: str):
    with open(img_path, "rb") as f:
        hash = hashlib.file_digest(f, "sha256").hexdigest()
    with conn.cursor() as cur:
        cur.execute(
            "INSERT INTO photos (file_path, sha256) "
            "VALUES (%s, %s) ",
            (img_path, hash)
        )

def register(conn, image_info: list, img: str, name: str, notes: str="None"):
    with conn.cursor() as cur:
        result = image_info

        cur.execute(
            "INSERT INTO people (name, notes) VALUES (%s, %s) "
            "ON CONFLICT (name) DO UPDATE SET notes = EXCLUDED.notes "
            "RETURNING id",
            (name, notes)
        )

        abs_path = str(Path(img).resolve(strict=True))
        embedding = result[0]["embedding"]
        person_id = cur.fetchone()[0]
        add_embedding(conn=conn, person_id=person_id, embedding=embedding, img_path=abs_path)

        add_photo(conn=conn, img_path=abs_path)

        print(f"{name} added successfully!")

with psycopg.connect("postgresql://localhost/photos") as conn:
    register_vector(conn)
    result = image_info("../IMG_8882.jpg")
    register(conn, result, "../IMG_8882.jpg", "Hunter")
