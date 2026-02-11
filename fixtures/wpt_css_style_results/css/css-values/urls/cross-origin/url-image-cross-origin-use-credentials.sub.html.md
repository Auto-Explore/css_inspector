# css/css-values/urls/cross-origin/url-image-cross-origin-use-credentials.sub.html

```json
{
  "format_version": 3,
  "file": "css/css-values/urls/cross-origin/url-image-cross-origin-use-credentials.sub.html"
}
```

## style[0]

```css

  .test {
    width: 200px;
    height: 200px;
    background-color: red;
    background-image: url("http://{{hosts[][]}}:{{ports[http][1]}}/css/support/1x1-green.png?pipe=header(Access-Control-Allow-Origin,http://{{hosts[][]}}:{{ports[http][0]}})|header(Access-Control-Allow-Credentials,true)" cross-origin(use-credentials));
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
