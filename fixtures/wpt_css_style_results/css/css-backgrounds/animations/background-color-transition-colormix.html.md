# css/css-backgrounds/animations/background-color-transition-colormix.html

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/animations/background-color-transition-colormix.html"
}
```

## style[0]

```css

    #parent {
      color: white;
      background-color:
          color-mix(in srgb,
                    color-mix(in srgb, black, currentColor),
                    black);
      height: 200px;
      width: 200px;
    }
    #child {
      background-color: inherit;
      transition: background-color;
      height: 100px;
      width: 100px;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
