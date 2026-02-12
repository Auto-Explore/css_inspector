# css/css-images/object-fit-dyn-aspect-ratio-001.html

```json
{
  "format_version": 3,
  "file": "css/css-images/object-fit-dyn-aspect-ratio-001.html"
}
```

## style[0]

```css

      object {
        margin: 1px;
        float: left;
        /* I'm just using 'object-position' for cosmetic reasons, so that the
           painted areas are all snapped to top-left which makes reference case
           more trivial. */
        object-position: top left;
      }
      .cov { object-fit: cover;   }
      .con { object-fit: contain; }

      .square {
        width: 24px;
        height: 24px;
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

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
