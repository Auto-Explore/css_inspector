# css/css-transforms/backface-visibility-hidden-animated-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/backface-visibility-hidden-animated-ref.html"
}
```

## style[0]

```css

    #flip {
      height: 100px;
      width: 100px;
      transform: rotateY(180deg);
      transform-style: preserve-3d;
    }

    #back {
      background: lightblue;
      transform: rotateY(180deg);
      position: absolute;
      top: 0;
      left: 0;
      width: 200px;
      height: 200px;
      backface-visibility: hidden;
    }

    #posabs {
      position: absolute;
      bottom: 0;
      right: 0;
      background: yellow;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
