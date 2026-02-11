# css/css-masking/mask-image/mask-clip-1.html

```json
{
  "format_version": 3,
  "file": "css/css-masking/mask-image/mask-clip-1.html"
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

      div.border {
        left: 10px;
        mask-clip: border-box;
      }

      div.padding {
        left: 110px;
        mask-clip: padding-box;
      }

      div.content {
        left: 210px;
        mask-clip: content-box;
      }

      div.no-clip {
        background-color: yellow;
        left: 310px;
        mask-clip: no-clip;
      }

      div.no-clip-inner {
        background-color: purple;
        position: absolute;
        /* allign with border area of the parent*/
        top: -8px;
        left: -6px;
        width: 100px;
        height: 50px;
      }

    
```

```json
{
  "errors": 7,
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
    },
    {
      "message": "Unknown property “mask-clip”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
