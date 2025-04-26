# Replicate Server

**Version:** 0.1.0  
**Total Tools:** 7

<details>
<summary><strong>ListModels</strong></summary>

**Description:** List models from Replicate (with optional name filter).

**Parameters:**
- `name_filter`: Optional substring to match in model name
- `limit`: Max results to return (default 10)

</details>

<details>
<summary><strong>GenerateImage</strong></summary>

**Description:** Generate an image via Replicate by prompting the model.

**Parameters:**
- `model_id`: ID of the model, e.g. 'black-forest-labs/flux-1.1-pro-ultra'
- `prompt`: Text prompt for generation
- `lora_weights`: Optional LoRA weights to apply, e.g. 'fofr/flux-80s-cyberpunk'

</details>

<details>
<summary><strong>EditImage</strong></summary>

**Description:** Edit an image with a prompt.

**Parameters:**
- `image`: URL of the image to edit
- `prompt`: Text prompt for guiding the edit
- `steps`: Number of steps for image generation (default: 28)
- `guidance`: Guidance scale for the model (default: 25)

</details>

<details>
<summary><strong>EditImageWithMask</strong></summary>

**Description:** Edit an image with a mask and prompt.

**Parameters:**
- `image`: URL of the image to edit
- `mask`: URL of the mask image
- `prompt`: Text prompt for guiding the edit
- `steps`: Number of steps for image generation (default: 28)
- `guidance`: Guidance scale for the model (default: 25)

</details>

<details>
<summary><strong>GetPrediction</strong></summary>

**Description:** Get prediction results from Replicate.

**Parameters:**
- `prediction_id`: The ID of the prediction to fetch

</details>

<details>
<summary><strong>GetModelInfo</strong></summary>

**Description:** Get information about a specific model on Replicate.

**Parameters:**
- `model_id`: The ID of the model to fetch info for

</details>

<details>
<summary><strong>Whoami</strong></summary>

**Description:** Get information about the current Replicate user.

**Parameters:**
_None_

</details> 