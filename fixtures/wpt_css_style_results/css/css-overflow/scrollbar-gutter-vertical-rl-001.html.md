# css/css-overflow/scrollbar-gutter-vertical-rl-001.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scrollbar-gutter-vertical-rl-001.html"
}
```

## style[0]

```css

  .line {
    display: flex;
  }

  .container {
    writing-mode: vertical-rl;

    height: 200px;
    width: 200px;
    margin: 10px;
    padding: 0px;
    border: none;
    overflow-y: auto;
    background: deepskyblue;
  }

  .content {
    width: 100%;
    height: 100%;
    padding: 0px;
    border: none;
    background: lightsalmon;
  }

  /* scrollbar-gutter */
  .sg_auto {
    scrollbar-gutter: auto;
  }

  .sg_stable {
    scrollbar-gutter: stable;
  }

  .sg_stable_mirror {
    scrollbar-gutter: stable both-edges;
  }

  /* overflow */
  .container.ov_auto {
    overflow-x: auto;
  }

  .container.ov_scroll {
    overflow-x: scroll;
  }

  .container.ov_visible {
    overflow: visible;
  }

  .container.ov_hidden {
    overflow-x: hidden;
  }

  .container.ov_clip {
    overflow: clip;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “scrollbar-gutter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
