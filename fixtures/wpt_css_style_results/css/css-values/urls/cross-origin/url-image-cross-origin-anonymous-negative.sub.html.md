# css/css-values/urls/cross-origin/url-image-cross-origin-anonymous-negative.sub.html

```json
{
  "format_version": 3,
  "file": "css/css-values/urls/cross-origin/url-image-cross-origin-anonymous-negative.sub.html"
}
```

## style[0]

```css

  .test {
    width: 200px;
    height: 200px;
    background-color: green;
    background-image: url("http://{{hosts[][]}}:{{ports[http][1]}}/css/support/1x1-red.png" cross-origin(anonymous));
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
