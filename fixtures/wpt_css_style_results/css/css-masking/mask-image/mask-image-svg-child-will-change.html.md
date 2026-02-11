# css/css-masking/mask-image/mask-image-svg-child-will-change.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-image-svg-child-will-change.html"
}
```

## style[0]

```css

svg {
  /* The image is 200x200 with 100x100 opaque pixels at the center */
  -webkit-mask-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMgAAADICAYAAACtWK6eAAABNklEQVR42u3TyQ0AMAgEMUj/PS895IWQ3QHHVAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP/6yiBJ4pyLHqv7xG89pwSBgEBAICAQEAgIBAQCAgGBAAIBgYBAQCAgEBAICAQEAgIBgQACAYGAQEAgIBAQCAgEBAICAQQCAgGBgEBAICAQEAgIBAQCAgEEAgIBgYBAQCAgEBAICAQEAggEBAICAYGAQEAgIBAQCAgEBAIIBAQCAgGBgEBAICAQEAgIBBAICAQEAgIBgYBAQCAgEBAICAQQCAgEBAICAYGAQEAgIBAQCAjECkAgIBAQCAgEBAICAYGAQEAggEBAICAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAthrrbgTIalbwsQAAAABJRU5ErkJggg==);
  mask-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAMgAAADICAYAAACtWK6eAAABNklEQVR42u3TyQ0AMAgEMUj/PS895IWQ3QHHVAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAP/6yiBJ4pyLHqv7xG89pwSBgEBAICAQEAgIBAQCAgGBAAIBgYBAQCAgEBAICAQEAgIBgQACAYGAQEAgIBAQCAgEBAICAQQCAgGBgEBAICAQEAgIBAQCAgEEAgIBgYBAQCAgEBAICAQEAggEBAICAYGAQEAgIBAQCAgEBAIIBAQCAgGBgEBAICAQEAgIBBAICAQEAgIBgYBAQCAgEBAICAQQCAgEBAICAYGAQEAgIBAQCAjECkAgIBAQCAgEBAICAYGAQEAggEBAICAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAthrrbgTIalbwsQAAAABJRU5ErkJggg==);
  width: 100px;
  height: 100px;
  border: 50px solid red;
  background: green;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “-webkit-mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
