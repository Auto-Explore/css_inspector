# css/css-page/page-rule-declarations-003.html

```json
{
  "format_version": 3,
  "file": "css/css-page/page-rule-declarations-003.html"
}
```

## style[0]

```css

    @page a, B {
        size: 1in;
    }
    @page A,b,C {
        size: 2in;
    }
    @page auto {
        size: 3in;
    }
    @page something, auto {
        size: 4in;
    }
    @page auto, other_thing {
        size: 5in;
    }
    @page _a, Z {
        size: 6in;
    }
    @page -b, y {
        size: 7in;
    }
    @page _abcd {
        size: 8in;
    }
    @page n,-XYZ {
        size: 9in;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
