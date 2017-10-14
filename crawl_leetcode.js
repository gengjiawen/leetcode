const Sequelize = require('sequelize')
const axios = require('axios')

const sequelize = new Sequelize('sqlite://leetcode.db', {
  define: {
    timestamps: true,
    freezeTableName: true,
    logging: false
  }
})

const Problem = sequelize.define('leetcode', {
  id: {type: Sequelize.INTEGER, primaryKey: true},
  url: {type: Sequelize.STRING},
  title: {type: Sequelize.STRING},
  difficulty: {type: Sequelize.INTEGER},
  paidOnly: {type: Sequelize.BOOLEAN}
})

Problem.sync()

async function saveRecord (url) {
  const r = await axios.get('https://leetcode.com/api/problems/algorithms/')
  const response = r.data
  response.stat_status_pairs.forEach(item => {
    console.log(item)
    Problem.upsert({
      id: item.stat.question_id,
      url: `https://leetcode.com/problems/${item.stat.question__title_slug}`,
      title: item.stat.question__title,
      difficulty: item.difficulty.level,
      paidOnly: item.paid_only
    })
  })

  return response
}

saveRecord()
  .then(r => {
    console.log(r)
  })
  .catch(e => {
    console.log(e)
  })

async function getBody (url, force = false) {
  if (force) {
    return await saveRecord(url)
  }

  const a = await Problem.findOne({where: {url: url}})
  if (a !== null) {
    return a.dataValues.body
  }

  return await saveRecord(url)
}
