# css/css-fonts/variations/at-font-face-font-matching.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/at-font-face-font-matching.html"
}
```

## style[0]

```css

        .test
        {
            float:left;
            border:1px solid red;
            font-size:24pt;
            white-space: nowrap;
            clear:both;
        }

        @font-face { font-family: W100; src: url('./resources/csstest-weights-100-kerned.ttf'); }
        @font-face { font-family: W200; src: url('./resources/csstest-weights-200-kerned.ttf'); }
        @font-face { font-family: W300; src: url('./resources/csstest-weights-300-kerned.ttf'); }


        @font-face { font-family: descriptorPriorityTest; src: url('./resources/csstest-weights-100-kerned.ttf'); font-stretch: 125%; }
        @font-face { font-family: descriptorPriorityTest; src: url('./resources/csstest-weights-200-kerned.ttf'); font-style: italic; }
        @font-face { font-family: descriptorPriorityTest; src: url('./resources/csstest-weights-300-kerned.ttf'); font-weight: 350; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
