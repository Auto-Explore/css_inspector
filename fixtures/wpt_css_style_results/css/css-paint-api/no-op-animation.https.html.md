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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
