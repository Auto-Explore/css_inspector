# css/css-anchor-position/anchor-size-css-zoom.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-size-css-zoom.html"
}
```

## style[0]

```css

    #containing-block {
        position: relative;

        zoom: 2;
    }

    #anchor {
        position: absolute;

        width: 200px;
        height: 100px;

        anchor-name: --anchor;

        background: red;
    }

    #anchor-positioned {
        position: absolute;

        width: anchor-size(--anchor width);
        height: anchor-size(--anchor height);

        background: green;

        z-index: 1;
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
