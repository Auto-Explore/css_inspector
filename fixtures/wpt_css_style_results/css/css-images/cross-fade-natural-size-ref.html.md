# css/css-images/cross-fade-natural-size-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/cross-fade-natural-size-ref.html"
}
```

## style[0]

```css

      div {
        margin: 2px;
      }
      .div1::before {
        content: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='375' height='350' style='background: black'><rect fill='red' width='140.625' height='131.25' fill-opacity='0.75'/><rect fill='blue' width='187.5' height='262.5' fill-opacity='0.25' style='mix-blend-mode: screen'/></svg>");
      }
      .div2::before {
        content: url("data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' width='375' height='350' style='background: black'><rect fill='red' width='140.625' height='131.25' fill-opacity='0.375'/><rect fill='blue' width='187.5' height='262.5' fill-opacity='0.125' style='mix-blend-mode: screen'/><rect fill='green' width='375' height='350' fill-opacity='0.5' style='mix-blend-mode: screen'/></svg>");
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
