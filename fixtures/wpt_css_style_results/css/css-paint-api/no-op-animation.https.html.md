# css/css-paint-api/no-op-animation.https.html

```json
{
  "format_version": 3,
  "file": "css/css-paint-api/no-op-animation.https.html"
}
```

## style[0]

```css

#container {
}
.animate {
  background-image: paint(foo);
  animation: anim 1s;
}
@keyframes anim {
  0% { --foo: rgb(200, 0, 0); }
}
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
