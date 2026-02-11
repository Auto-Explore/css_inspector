# css/css-page/cssom/margin-001.html

```json
{
  "format_version": 3,
  "file": "css/css-page/cssom/margin-001.html"
}
```

## style[0]

```css

  @page {}
  @page {
    @top-center {}
  }
  @page {
    color: red;

    /* This property doesn't apply, but the declaration should still be
       included: */
    column-count: 7;

    @bottom-left {
      margin: inherit;
    }
    @top-right {
      content: "hot";
      font-size: 2em;
    }
    @top-left-corner {}
    @top-left {}
    @top-center {}
    @top-right {}
    @top-right-corner {}

    color: inherit;

    @right-top {}
    @right-middle {}
    @right-bottom {}
    @bottom-right-corner {}
    @bottom-right {}
    @bottom-center {}
    @bottom-left {}
    @bottom-left-corner {}
    @left-bottom {}
    @left-middle {}
    @left-top {}
    @herring {}

    margin-left: 111px;
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
