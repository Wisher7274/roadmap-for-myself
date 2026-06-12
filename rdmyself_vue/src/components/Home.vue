<script setup>
</script>

<template>
  <div class="page">
    <header class="header">
      <span>Wisher7274's blog & roadmap & 💗</span>
    </header>

    <main class="content">
      <h1 class="title">Wisher7274</h1>

      <!-- section = 页面里的一块独立区域，这里专门放「自我介绍」 -->
      <section class="intro">
        <h2 class="intro-title">自我介绍/About me</h2>

        <!-- 中文一段、英文一段，各用一个 p，不要挤在同一个标签里 -->
        <p class="intro-zh">嗨～我是一名刚高考完的毕业生，对开发很有兴趣，了解过Linux，也学了Godot，还在GitHub上放了一个简陋的类galgame剧情播放器（算是我的第一个项目）。喜欢Neuro-sama，希望AI陪伴能真正打破次元壁，变得有生命力！目前我懂的还很零碎，没什么工程经验，也没有成型的编程思想，天天纠结学什么技术、走哪条路，假期只能断断续续地摸索 (｡•́︿•̀｡) 我性格闷骚又容易自卑，所以特别想通过学习路线和开发方法的摸索，刷新认知，更想锻炼自己和别人交流的能力。虽然暂时贡献不了什么代码，但我有满满的时间+热情，希望能跟着大家一起玩～</p>

        <p class="intro-en">Hi there~ I just finished my college entrance exams and I'm super into coding! I know a bit about Linux, learned Godot, and have a tiny galgame-style story player on GitHub (my little project). I love Neuro-sama and hope AI companions can truly break the fourth wall and feel alive! Right now my knowledge is still scattered—no real engineering experience, no solid programming mindset. I keep stressing over which tech stack to learn and which path to take, and I can only learn in bits during holidays (｡•́︿•̀｡). I'm a bit shy and easily get down on myself, so I really want to explore the right learning path, understand how to develop things properly, and most importantly, practice talking with other devs. I can't contribute much code yet, but I have lots of time and tons of passion, so I'd love to join and have fun together~</p>

        <p class="intro-contact">如果你也想和我一起玩，欢迎在GitHub上找我：<a href="https://github.com/Wisher7274" target="_blank">Wisher7274</a>！</p>
      </section>
    </main>

    <footer class="footer">
      <span>&copy; 2026 Jacob Lee. All rights reserved.</span>
      <p>Made by Wisher7274 with 💗</p>
    </footer>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}
.header {
  background-color: #333;
  color: #fff;
  /* 上下 + 左右留白，文字不贴顶栏边缘 */
  padding: 1rem 1.5rem;
  text-align: left;
}
.content {
  flex: 1;
  max-width: 42rem;
  margin: 0 auto;
  text-align: left;
  padding: 2rem;
}

.title {
  text-align: center;
  margin-bottom: 1.5rem;
}

/* 「自我介绍」整块：浅底 + 圆角 + 内边距，和外面背景区分开 */
.intro {
  background: var(--color-background-soft);
  border: 1px solid var(--color-border);
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.intro-title {
  text-align: center;
  margin-bottom: 1rem;
  font-size: 1.25rem;
}

.intro p {
  line-height: 1.75;
  margin-bottom: 0;
}

/* 英文段上方加间距 + 细线，和中英文视觉上分开 */
.intro-en {
  margin-top: 1.25rem;
  padding-top: 1.25rem;
  border-top: 1px solid var(--color-border);
}

/* 联系方式单独居中，和上面正文区分开 */
.intro-contact {
  margin-top: 1.25rem;
  text-align: center;
}

.footer {
  background-color: #333;
  color: #fff;
  /* 上下 + 左右留白，文字不贴底栏边缘 */
  padding: 1rem 1.5rem;
  text-align: center;
  font-size: 0.875rem;
  opacity: 0.8;
}
</style>
