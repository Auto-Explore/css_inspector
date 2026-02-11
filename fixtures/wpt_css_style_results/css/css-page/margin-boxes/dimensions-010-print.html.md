# css/css-page/margin-boxes/dimensions-010-print.html

```json
{
  "format_version": 3,
  "file": "css/css-page/margin-boxes/dimensions-010-print.html"
}
```

## style[0]

```css

  :root {
    print-color-adjust: exact;
  }
  @page {
    margin: 100px;
    size: 650px 500px;

    @top-left {
      background: hotpink;
      content: "";
    }
    @top-center {
      background: cyan;
      content: "";
    }
    @top-right {
      background: yellow;
      content: "";
    }

    @left-top {
      background: yellow;
      content: "";
    }
    @left-middle {
      background: cyan;
      content: "";
    }
    @left-bottom {
      background: hotpink;
      content: "";
    }

    @right-top {
      background: red;
      content: "";
    }
    @right-middle {
      background: cyan;
      content: "\a0";
    }
    @right-bottom {
      background: red;
      content: "";
    }

    @bottom-left {
      background: hotpink;
      content: "\a0";
    }
    @bottom-center {
      background: red;
      content: "";
    }
    /* Since there's a center box here, although it doesn't require any space,
       it will be there right in the center, meaning that the hotpink box, which
       *does* require space, can only receive everything to the left of the
       center box. */
    @bottom-right {
      background: yellow;
      content: "";
    }
  }
```

```json
{
  "errors": 7,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
