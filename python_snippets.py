#get member by name or id etc
member = list(filter(
    lambda member: username in member.name, 
    ctx.guild.members
))[0]
