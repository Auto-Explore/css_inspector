# css/css-images/tools/template-object-fit-test.html

```json
{
  "format_version": 3,
  "file": "css/css-images/tools/template-object-fit-test.html"
}
```

## style[0]

```css

      REPLACEME_CONTAINER_TAG {
        border: 1px dashed gray;
        padding: 1px;
        object-fit: REPLACEME_OBJECT_FIT_VAL;
        image-rendering: crisp-edges;
        float: left;
      }

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

      .tr { object-position: top right }
      .bl { object-position: bottom left }
      .tl { object-position: top 25% left 25% }
      .br { object-position: bottom 1px right 2px }

      .tc { object-position: top 3px left 50% }
      .cr { object-position: top 50% right 25% }
    
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “object-position”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
