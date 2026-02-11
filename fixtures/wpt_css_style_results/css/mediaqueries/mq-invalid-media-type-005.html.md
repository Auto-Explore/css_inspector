# css/mediaqueries/mq-invalid-media-type-005.html

```json
{
  "format_version": 3,
  "file": "css/mediaqueries/mq-invalid-media-type-005.html"
}
```

## style[0]

```css

    @media not and {
        div { background-color: red; }
    }
    @media and {
        div { background-color: red; }
    }
    @media not or {
        div { background-color: red; }
    }
    @media or {
        div { background-color: red; }
    }
    @media not not {
        div { background-color: red; }
    }
    @media not {
        div { background-color: red; }
    }
    @media not only {
        div { background-color: red; }
    }
    @media only {
        div { background-color: red; }
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
