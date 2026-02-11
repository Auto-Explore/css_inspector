# css/css-lists/inherit-overwrites.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/inherit-overwrites.html"
}
```

## style[0]

```css

  #container {
    counter-reset: first 1;
    counter-increment: second 2;
    counter-set: third 3;
  }
  .target {
    counter-reset: fourth 4;
    counter-increment: fifth 5;
    counter-set: sixth 6;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “counter-set”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
