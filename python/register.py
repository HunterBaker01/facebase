from deepface import DeepFace

def enrol(img: str, img_name: str):
    DeepFace.register(
        img=img,
        img_name=img_name,
        model_name="ArcFace",
        database_type="postgres",
        connection_details="postgres://localhost/family_photos"
    )

enrol("../IMG_8882.jpg", "Hele")
