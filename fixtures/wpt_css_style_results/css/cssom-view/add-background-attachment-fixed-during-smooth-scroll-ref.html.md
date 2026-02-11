# css/cssom-view/add-background-attachment-fixed-during-smooth-scroll-ref.html

```json
{
  "format_version": 3,
  "file": "css/cssom-view/add-background-attachment-fixed-during-smooth-scroll-ref.html"
}
```

## style[0]

```css

#container {
  width: 200px;
  height: 200px;
  overflow: scroll;
  background: linear-gradient(green, blue);
  background-attachment: fixed;
}
#content {
  width: 7500px;
  height: 7500px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
