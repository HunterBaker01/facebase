from deepface import DeepFace
import json
import sys

def embeddings(img: str):
    result = DeepFace.represent(
        img_path=img,
        model_name="ArcFace",
        detector_backend="retinaface",
        l2_normalize=True,
    )

    print(json.dumps(result))

if __name__ == "__main__":
    image_path = sys.argv[1]

    try:
        embeddings(img=image_path)
    except Exception as e:
        print(f"error", json.dumps(str(e)))
        sys.exit(1)
