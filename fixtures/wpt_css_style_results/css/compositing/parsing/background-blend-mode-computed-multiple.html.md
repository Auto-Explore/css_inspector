# css/compositing/parsing/background-blend-mode-computed-multiple.html

```json
{
  "format_version": 3,
  "file": "css/compositing/parsing/background-blend-mode-computed-multiple.html"
}
```

## style[0]

```css

    #target {
        background-image: linear-gradient(green, green), linear-gradient(green, green), linear-gradient(green, green);
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
