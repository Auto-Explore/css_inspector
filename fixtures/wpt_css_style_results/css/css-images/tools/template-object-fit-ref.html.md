# css/css-images/tools/template-object-fit-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/tools/template-object-fit-ref.html"
}
```

## style[0]

```css

      .objectOuter {
        border: 1px dashed gray;
        padding: 1px;
        float: left;
      }
      .objectOuter > * {
        background-image: url("REPLACEME_IMAGE_FILENAME");
        background-size: REPLACEME_BACKGROUND_SIZE_VAL;
        background-repeat: no-repeat;
        image-rendering: pixelated; /* for UAs that don't support crisp-edges */
        image-rendering: crisp-edges;
      }
      REPLACEME_SCALE_DOWN_EXTRA_RULE
      .bigWide {
        width: 48px;
        height: 32px;
      }
      .bigTall {
        width: 32px;
        height: 48px;
      }
      .small {
        width: 8px;
        height: 8px;
      }

      br { clear: both; }

      .tr { background-position: top right }
      .bl { background-position: bottom left }
      .tl { background-position: top 25% left 25% }
      .br { background-position: bottom 1px right 2px }

      .tc { background-position: top 3px center }
      .cr { background-position: center right 25% }
      .default { background-position: 50% 50% }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
