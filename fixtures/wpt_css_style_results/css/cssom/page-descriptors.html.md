# css/cssom/page-descriptors.html

```json
{
  "format_version": 3,
  "file": "css/cssom/page-descriptors.html"
}
```

## style[0]

```css

@page {
    size: a3;
    page-orientation: rotate-right;
    margin: 1em 24px 2in 101.5mm;
}
@page {
    size: jis-b5 landscape;
}
@page {
    size: 216mm;
}
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “page-orientation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
