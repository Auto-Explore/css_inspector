# css/css-transforms/perspective-transforms-equivalence-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/perspective-transforms-equivalence-ref.html"
}
```

## style[0]

```css


#container {
  transform: translate(-200px, -200px);
  width: 500px;
  height: 500px;
  perspective: 500px;
}

#container > div {
  width: 100%;
  height: 100%;
  transform-style: preserve-3d;
  transform: translateZ(-250px) rotateZ(45deg);
}

#container > div > div {
  width: 100%;
  height: 100%;
  position: absolute;
  background-color: rgba(3, 121, 255, 0.3);
  box-sizing: border-box;
  border: 3px solid black;
}

#one { transform: rotateY(90deg)  translateZ(250px); }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
