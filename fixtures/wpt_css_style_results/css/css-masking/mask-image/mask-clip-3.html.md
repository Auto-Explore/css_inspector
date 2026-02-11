# css/css-masking/mask-image/mask-clip-3.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-3.html"
}
```

## style[0]

```css

      div.outer {
        /*
         * content box: 40 x 20
         * padding box: 52 x 38
         * border  box: 60 x 50
         * margin  box: 66 x 54
         */
        background-color: purple;
        position: absolute;
        top: 10px;
        margin: 1px 2px 3px 4px;
        border: solid transparent;
        border-width: 8px 2px 4px 6px;
        padding: 6px 9px 12px 3px;
        width: 40px;
        height: 20px;
      }

      div.mask {
        mask-size: 100% 100%;
        mask-origin: border-box;
        mask-image: url(support/transparent-100x50-blue-100x50.svg);
      }

      div.stroke {
        left: 10px;
        mask-clip: stroke-box; /* should be the same as border-box */
      }

      div.view {
        left: 110px;
        mask-clip: view-box; /* should be the same as border-box */
      }

      div.fill {
        left: 210px;
        mask-clip: fill-box; /* should be the same as content-box */
      }
    
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown property “mask-size”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-origin”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-image”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
