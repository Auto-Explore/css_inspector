# css/css-anchor-position/container-queries/anchored-fallback-name.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-name.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
  }
  @position-try --foo {
    position-area: bottom;
  }
  @position-try --bar {
    position-area: top;
  }
  .anchored {
    position: absolute;
    position-anchor: --a;
    position-area: top;
    width: 100px;
    /* Too tall to fit over the anchor to trigger fallback */
    height: 100px;
    container-type: anchored;
  }
  #a1 {
    position-try-fallbacks: --foo;
  }
  #t1 {
    @container anchored(fallback: --foo) { --pass: yes; }
  }
  #a2 {
    position-try-fallbacks: --bar, --bar flip-block;
  }
  #t2 {
    @container anchored(fallback: --bar flip-block) { --pass: yes; }
    @container anchored(fallback: --bar) { --pass: no; }
  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
