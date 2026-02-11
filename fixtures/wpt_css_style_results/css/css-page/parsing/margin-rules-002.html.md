# css/css-page/parsing/margin-rules-002.html

```json
{
  "format_version": 3,
  "file": "css/css-page/parsing/margin-rules-002.html"
}
```

## style[0]

```css

  @page {
    @top-center {
      content: "PASS";
    }
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

## style[1]

```css

  @page {
    @top-center {
      content: "PASS";
    }
    padding-left: 10px;
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

## style[2]

```css

  @page {
    padding-left: 10px;
    @top-center {
      content: "PASS";
    }
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

## style[3]

```css

  @page {
    padding-left: 666px;
    @top-left {
      content: "left";
    }
    padding-left: 10px;
    @top-right {
      content: "right";
    }
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
