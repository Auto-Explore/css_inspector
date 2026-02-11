# css/css-values/if-media-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-media-invalidation.html"
}
```

## style[0]

```css

  iframe {
    width: 50px;
    height: 50px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    #target {
      --actual: if(media((height < 100px) or ((height >= 200px) and (height < 300px))): true_value; else: false_value;);
    }
  
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “else”.",
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
