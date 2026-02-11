# css/css-anchor-position/container-queries/anchored-fallback-try-tactic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/container-queries/anchored-fallback-try-tactic.html"
}
```

## style[0]

```css

  #anchor {
    anchor-name: --a;
    width: 100px;
    height: 100px;
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
    position-try-fallbacks: flip-block;
  }
  #a2 {
    position-try-fallbacks: flip-inline flip-block;
  }
  #a3 {
    position-try-fallbacks: flip-x flip-y;
  }
  #t1, #t2, #t3 {
    --flip-block: no;
    --flip-inline: no;
    --flip-inline-block: no;
    --flip-block-inline: no;
    --flip-x: no;
    --flip-y: no;
    --flip-x-y: no;
    --flip-y-x: no;
    @container anchored(fallback: flip-block) { --flip-block: yes; }
    @container anchored(fallback: flip-inline) { --flip-inline: yes; }
    @container anchored(fallback: flip-inline flip-block) { --flip-inline-block: yes; }
    @container anchored(fallback: flip-block flip-inline) { --flip-block-inline: yes; }
    @container anchored(fallback: flip-x) { --flip-x: yes; }
    @container anchored(fallback: flip-y) { --flip-y: yes; }
    @container anchored(fallback: flip-x flip-y) { --flip-x-y: yes; }
    @container anchored(fallback: flip-y flip-x) { --flip-y-x: yes; }
  }
```

```json
{
  "errors": 6,
  "messages": [
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
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
