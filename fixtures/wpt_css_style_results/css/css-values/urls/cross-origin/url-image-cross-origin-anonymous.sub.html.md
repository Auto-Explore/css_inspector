# css/css-values/urls/cross-origin/url-image-cross-origin-anonymous.sub.html

```json
{
  "format_version": 3,
  "file": "css/css-values/urls/cross-origin/url-image-cross-origin-anonymous.sub.html"
}
```

## style[0]

```css

  .test {
    width: 200px;
    height: 200px;
    background-color: red;
    background-image: url("http://{{hosts[][]}}:{{ports[http][1]}}/css/support/1x1-green.png?pipe=header(Access-Control-Allow-Origin,*)" cross-origin(anonymous));
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
