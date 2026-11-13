import json
import sys

from deepface import DeepFace


def embeddings(img: str):
    result = DeepFace.represent(
        img_path=img,
        model_name="ArcFace",
        detector_backend="retinaface",
        l2_normalize=True,
        enforce_detection=False,
    )

    faces = []

    for face in result:
        if face["face_confidence"] >= 0.8:
            faces.append(face)

    if len(faces) < 1:
        print("No faces detected...\nPlease upload a valid image...", file=sys.stderr)
        sys.exit(1)
    elif len(faces) > 1:
        print(
            "Too many people...\nOnly one person in the image please...",
            file=sys.stderr,
        )
        sys.exit(1)

    print(json.dumps(faces[0]))


if __name__ == "__main__":
    image_path = sys.argv[1]

    try:
        embeddings(img=image_path)
    except Exception as e:
        print(f"{e}", file=sys.stderr)
        sys.exit(1)
