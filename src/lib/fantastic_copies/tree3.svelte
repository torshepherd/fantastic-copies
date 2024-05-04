<script lang="ts">
  import Step from '@lib/components/step.svelte'
  import { signal } from '@motion'

  const animations = signal(
    { op1: 0, op2: 0, op3: 0, op4: 0, op5: 0, op6: 0, op7: 0 },
    { duration: 500 }
  )
</script>

<div class="grow flex gap-4 justify-center">
  <svg
    class="w-[800px] h-[280px] mt-[60px] mx-auto stroke-white stroke-1 fill-transparent text-sm"
  >
    <rect x="0" y="0" width="200" height="400" stroke="none" fill="#006466" />
    <text
      x="100"
      y="10"
      stroke="none"
      fill="white"
      dominant-baseline="hanging"
      text-anchor="middle"
      font-size="18"
    >
      Rebindings
    </text>
    <g transform="translate(100, 90)">
      <rect x="-50" y="30" width="100" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        RVO
      </text>
      <rect x="-50" y="80" width="100" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="100"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="14"
      >
        <tspan x="0" y="93">Init from</tspan>
        <tspan x="0" y="107">temporary</tspan>
      </text>
      <!-- <rect
        x="-50"
        y="130"
        width="100"
        height="40"
        rx="10"
        fill="transparent"
      />
      <text
        x="0"
        y="150"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        emplace
      </text> -->
    </g>
    <rect x="200" y="0" width="600" height="400" stroke="none" fill="#3A2C53" />
    <text
      x="500"
      y="10"
      stroke="none"
      fill="white"
      dominant-baseline="hanging"
      text-anchor="middle"
      font-size="18"
    >
      Copies
    </text>
    <rect
      x="200"
      y="40"
      width="400"
      height="360"
      stroke="none"
      fill="#273F5A"
    />
    <text
      x="400"
      y="50"
      stroke="none"
      fill="white"
      dominant-baseline="hanging"
      text-anchor="middle"
      font-size="18"
    >
      Shallow (memcpy-like)
    </text>
    <rect
      x="200"
      y="80"
      width="200"
      height="320"
      stroke="none"
      fill="#135160"
    />
    <text
      x="300"
      y="90"
      stroke="none"
      fill="white"
      dominant-baseline="hanging"
      text-anchor="middle"
      font-size="18"
    >
      Bulk, coalescing
    </text>
    <g transform="translate(300, 90)">
      <rect x="-75" y="30" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="14"
      >
        trivially_relocatable
      </text>
      <rect x="-75" y="80" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="100"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="14"
      >
        trivially_copyable
      </text>
      <rect
        x="-75"
        y="130"
        width="150"
        height="40"
        rx="10"
        fill="transparent"
      />
      <text
        x="0"
        y="150"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        Scalar copies
      </text>
    </g>
    <rect
      x="600"
      y="40"
      width="200"
      height="360"
      stroke="none"
      fill="#4D194D"
    />
    <text
      x="700"
      y="50"
      stroke="none"
      fill="white"
      dominant-baseline="hanging"
      text-anchor="middle"
      font-size="18"
    >
      Deep (recursive)
    </text>
    <g transform="translate(600, 90)">
      <rect x="-75" y="30" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        throwing moves
      </text>
    </g>
    <g transform="translate(500, 140)">
      <rect x="-75" y="30" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        auto move
      </text>
    </g>
    <g transform="translate(500, 190)">
      <rect x="-75" y="30" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        std::move()
      </text>
    </g>
    <g transform="translate(700, 140)">
      <rect x="-75" y="30" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="16"
      >
        move-from-const
      </text>
    </g>
    <g transform="translate(700, 190)">
      <rect x="-75" y="30" width="150" height="40" rx="10" fill="transparent" />
      <text
        x="0"
        y="50"
        stroke="none"
        fill="white"
        dominant-baseline="middle"
        text-anchor="middle"
        font-size="18"
      >
        T(const T&)
      </text>
    </g>
  </svg>
</div>
